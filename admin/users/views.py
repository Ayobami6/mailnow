from django.shortcuts import render
from rest_framework.views import APIView
from sparky_utils.response import service_response

# Create your views here.


class RootAPIView(APIView):
    """
    Root Admin API View
    This view serves as the root endpoint for the API.
    It returns a simple message indicating that the API is working.
    """

    def get(self, request):

        return service_response(
            status="success",
            message="Admin Sevice is working",
            data=None,
            status_code=200,
        )
