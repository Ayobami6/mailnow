from django.contrib import admin
from .models import SMTPProfile, Template, Webhook, EmailLog


@admin.register(SMTPProfile)
class SMTPProfileAdmin(admin.ModelAdmin):
    list_display = ('smtp_server', 'company', 'smtp_username', 'smtp_port', 'is_default', 'created_at')
    list_filter = ('is_default', 'smtp_port', 'created_at')
    search_fields = ('smtp_server', 'company__company_name', 'smtp_username')
    raw_id_fields = ('company',)


@admin.register(Template)
class TemplateAdmin(admin.ModelAdmin):
    list_display = ('name', 'company', 'subject', 'template_type', 'date_created')
    list_filter = ('template_type', 'date_created')
    search_fields = ('name', 'company__company_name', 'subject')
    raw_id_fields = ('company',)


@admin.register(Webhook)
class WebhookAdmin(admin.ModelAdmin):
    list_display = ('name', 'company', 'url', 'status', 'is_active', 'created_at')
    list_filter = ('is_active', 'status', 'created_at')
    search_fields = ('name', 'company__company_name', 'url')
    raw_id_fields = ('company',)
    readonly_fields = ('last_delivered', 'success_rate')


@admin.register(EmailLog)
class EmailLogAdmin(admin.ModelAdmin):
    list_display = ('from_email', 'to_email', 'subject', 'status', 'company', 'created_at')
    list_filter = ('status', 'created_at')
    search_fields = ('from_email', 'to_email', 'subject')
    raw_id_fields = ('company',)
    readonly_fields = ('created_at',)
