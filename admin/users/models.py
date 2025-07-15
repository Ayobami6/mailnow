from django.db import models
from django.contrib.auth.models import (
    AbstractBaseUser,
    BaseUserManager,
    PermissionsMixin,
    AbstractUser,
)
from django.core.validators import validate_email, MinValueValidator, MaxValueValidator
from django.core.exceptions import ValidationError
from django.db import transaction
from django.utils.translation import gettext_lazy as _
from django.utils import timezone
from sparky_utils.decorators import str_meta
from .constants import Roles, APIKeyPermission

# Create your models here.


class CustomUserManger(BaseUserManager):

    def create_user(self, email, password=None, **extra_fields):
        """Creates and saves a new user"""
        if not email:
            raise ValueError("Users must have an email address")
        try:
            validate_email(email)
        except ValidationError:
            raise ValidationError({"email": _("Please enter a valid email.")})

        # start a singe unit operation
        with transaction.atomic():
            email = self.normalize_email(email)
            user = self.model(email=email, **extra_fields)
            user.set_password(password)
            user.save(using=self._db)
            return user

    def create_superuser(self, email, password=None, **extra_field):
        """Creates a new super user"""
        extra_field.setdefault("is_staff", True)
        extra_field.setdefault("is_superuser", True)
        return self.create_user(email, password, **extra_field)


class User(AbstractBaseUser, PermissionsMixin):
    email = models.EmailField(max_length=255, unique=True, db_index=True)
    firstname = models.CharField(max_length=255, null=True, blank=True)
    lastname = models.CharField(max_length=255, null=True, blank=True)
    is_active = models.BooleanField(default=True)
    is_staff = models.BooleanField(default=False)
    mfa_enabled = models.BooleanField(default=False)
    email_verifield = models.BooleanField(default=False)
    date_joined = models.DateTimeField(default=timezone.now)

    objects = CustomUserManger()

    USERNAME_FIELD = "email"

    def __str__(self):
        return self.email

    class Meta:
        db_table = "users"
        verbose_name = "User"
        verbose_name_plural = "Users"

    @property
    def full_name(self) -> str:
        return f"{self.first_name} {self.last_name}"

    @full_name.setter
    def full_name(self, first_name: str, last_name: str) -> None:
        self.first_name = first_name
        self.last_name = last_name
        self.save()


@str_meta
class Industry(models.Model):
    name = models.CharField(max_length=255, unique=True)
    description = models.TextField(blank=True, null=True)

    class Meta:
        db_table = "industries"
        verbose_name = "Industry"
        verbose_name_plural = "Industries"


@str_meta
class Company(models.Model):
    owner = models.OneToOneField(User, on_delete=models.CASCADE)
    company_name = models.CharField(max_length=255)
    company_address = models.TextField(blank=True, null=True)
    website = models.URLField(blank=True, null=True)
    sending_domain = models.CharField(max_length=255, blank=True, null=True)
    default_from_name = models.CharField(max_length=255, blank=True, null=True)
    default_from_email = models.EmailField(blank=True, null=True)
    industry = models.ForeignKey(Industry, on_delete=models.SET_NULL, null=True)

    class Meta:
        db_table = "companies"
        verbose_name = "Company"
        verbose_name_plural = "Companies"


class APIKey(models.Model):
    name = models.CharField(max_length=255)
    company = models.ForeignKey(Company, on_delete=models.CASCADE)
    api_key = models.CharField(max_length=255, unique=True)
    created_at = models.DateTimeField(auto_now_add=True)
    last_used = models.DateTimeField(blank=True, null=True)
    expires_at = models.DateTimeField(blank=True, null=True)
    is_active = models.BooleanField(default=True)
    permission = models.CharField(
        max_length=255,
        blank=True,
        null=True,
        choices=APIKeyPermission.choices(),
        default=APIKeyPermission.FULL_ACCESS.value,
    )

    def __str__(self):
        return self.api_key

    class Meta:
        db_table = "api_keys"
        verbose_name = "API Key"
        verbose_name_plural = "API Keys"


class TeamMember(models.Model):
    user = models.ForeignKey(User, on_delete=models.CASCADE)
    company = models.ForeignKey(Company, on_delete=models.CASCADE)
    role = models.CharField(
        max_length=255, choices=Roles.choices(), default=Roles.MEMBER.value
    )
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)

    def __str__(self):
        return self.user.email

    class Meta:
        db_table = "team_members"
        verbose_name = "Team Member"
        verbose_name_plural = "Team Members"
