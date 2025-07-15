from django.db import models
from django.contrib.postgres.fields import ArrayField
from users.constants import Status, EmailStatus
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

    def __str__(self):
        return self.name


class EmailLog(models.Model):
    from_email = models.CharField(max_length=200)
    to_email = models.CharField(max_length=200)
    subject = models.CharField(max_length=200)
    body = models.TextField()
    company = models.ForeignKey(Company, on_delete=models.CASCADE)
    status = models.CharField(
        max_length=255,
        blank=True,
        null=True,
        choices=EmailStatus.choices(),
        default=EmailStatus.SUCCESS.value,
    )
    created_at = models.DateTimeField(auto_now_add=True)

    def __str__(self):
        return f"{self.from_email} -> {self.to_email} : {self.subject[:50]} 📧"
