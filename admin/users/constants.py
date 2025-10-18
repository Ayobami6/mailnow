from enum import Enum
from typing import List, Tuple


class EnumBase(Enum):

    @classmethod
    def choices(cls) -> List[Tuple[str, str]]:
        return [(key.value, key.name) for key in cls]

    @classmethod
    def to_list(cls) -> List[str]:
        """convert the options to list

        Returns:
            List[str]: the options list
        """
        return [key.value for key in cls]


class APIKeyPermission(EnumBase):
    """API Key Permissions"""

    FULL_ACCESS = "Full Access"
    SEND_ONLY = "Send Only"
    READ_ONLY = "Read Only"
    WEBHOOK_ONLY = "Webhook Only"


class Status(EnumBase):
    """Status of the API Key"""

    ACTIVE = "Active"
    INACTIVE = "Inactive"


class Roles(EnumBase):
    OWNER = "Owner"
    ADMIN = "Admin"
    MEMBER = "Member"


class EmailStatus(EnumBase):
    FAILED = "Failed"
    SUCCESS = "Success"
    PENDING = "Pending"
    QUEUED = "Queued"
