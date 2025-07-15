from django.contrib import admin
from .models import User, APIKey

# Register your models here.


class UserAdmin(admin.ModelAdmin):
    list_display = [
        "email",
        "firstname",
        "lastname",
        "is_active",
        "is_staff",
        "is_superuser",
        "last_login",
        "date_joined",
    ]
    list_filter = ["is_active", "is_staff", "is_superuser"]
    search_fields = ["email", "firstname", "lastname"]


class APIKeyAdmin(admin.ModelAdmin):
    list_display = [
        "name",
        "company__company_name",
        "api_key",
        "permission",
        "created_at",
    ]
    list_filter = ["created_at", "permission"]
    search_fields = ["company__company_name", "api_key"]


admin.site.register(APIKey, APIKeyAdmin)
admin.site.register(User, UserAdmin)
