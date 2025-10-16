from django.contrib import admin
from django.contrib.auth.admin import UserAdmin as BaseUserAdmin
from .models import User, Company, Industry, APIKey, TeamMember


@admin.register(User)
class UserAdmin(BaseUserAdmin):
    list_display = ('email', 'firstname', 'lastname', 'is_active', 'email_verified', 'user_type', 'date_joined')
    list_filter = ('is_active', 'email_verified', 'user_type', 'is_staff', 'date_joined')
    search_fields = ('email', 'firstname', 'lastname')
    ordering = ('-date_joined',)
    
    fieldsets = (
        (None, {'fields': ('email', 'password')}),
        ('Personal info', {'fields': ('firstname', 'lastname', 'user_type')}),
        ('Permissions', {'fields': ('is_active', 'is_staff', 'is_superuser', 'email_verified', 'mfa_enabled')}),
        ('Important dates', {'fields': ('last_login', 'date_joined')}),
    )
    
    add_fieldsets = (
        (None, {
            'classes': ('wide',),
            'fields': ('email', 'password1', 'password2', 'firstname', 'lastname'),
        }),
    )


@admin.register(Industry)
class IndustryAdmin(admin.ModelAdmin):
    list_display = ('name', 'description')
    search_fields = ('name',)


@admin.register(Company)
class CompanyAdmin(admin.ModelAdmin):
    list_display = ('company_name', 'owner', 'website', 'industry', 'sending_domain')
    list_filter = ('industry',)
    search_fields = ('company_name', 'owner__email', 'website')
    raw_id_fields = ('owner',)


@admin.register(APIKey)
class APIKeyAdmin(admin.ModelAdmin):
    list_display = ('name', 'company', 'permission', 'is_active', 'created_at', 'last_used')
    list_filter = ('permission', 'is_active', 'created_at')
    search_fields = ('name', 'company__company_name')
    readonly_fields = ('api_key', 'created_at', 'last_used')
    raw_id_fields = ('company',)


@admin.register(TeamMember)
class TeamMemberAdmin(admin.ModelAdmin):
    list_display = ('user', 'company', 'role', 'created_at')
    list_filter = ('role', 'created_at')
    search_fields = ('user__email', 'company__company_name')
    raw_id_fields = ('user', 'company')
