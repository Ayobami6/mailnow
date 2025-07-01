from django.db import models
from django.contrib.postgres.fields import ArrayField
from users.constants import Status, APIKeyPermissions
from users.models import Company

# Create your models here.


class Webhook(models.Model):
    name = models.CharField(max_length=255)
    company = models.ForeignKey(Company, on_delete=models.CASCADE)
    url = models.URLField()
    created_at = models.DateTimeField(auto_now_add=True)
    updated_at = models.DateTimeField(auto_now=True)
    is_active = models.BooleanField(default=True)
    last_delivered = models.DateTimeField(blank=True, null=True)
    success_rate = models.FloatField(blank=True, null=True)
    events = ArrayField(models.CharField(max_length=255), blank=True, null=True)
    status = models.CharField(
        max_length=255,
        blank=True,
        null=True,
        choices=Status.choices(),
        default=Status.ACTIVE.value,
    )
    permission = models.CharField(
        max_length=255,
        blank=True,
        null=True,
        choices=APIKeyPermissions.choices(),
    )

    def __str__(self):
        return self.name
