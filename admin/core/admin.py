from django.contrib import admin
from .models import Webhook, EmailLog

# Register your models here.


class WehbookAdmin(admin.ModelAdmin):
    list_display = [
        "name",
        "company__company_name",
        "url",
        "status",
        "created_at",
        "is_active",
    ]
    list_filter = ["is_active", "company__company_name", "status"]
    search_fields = ["name", "company__company_name", "url"]


class EmailLogAdmin(admin.ModelAdmin):
    list_display = ["to_email", "from_email", "subject", "status", "created_at"]
    list_filter = ["status", "created_at"]
    search_fields = ["to_email", "subject", "status"]


admin.site.register(EmailLog, EmailLogAdmin)
admin.site.register(Webhook, WehbookAdmin)
