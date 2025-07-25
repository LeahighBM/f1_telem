from django.urls import path
from telem_app import views

urlpatterns = [
    path("headers/", views.header_list),
    path("headers/<int:id>/", views.single_header),
]