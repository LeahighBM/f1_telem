from rest_framework import serializers
from telem_app.models import TblHeaderPkts

class HeaderSerializer(serializers.ModelSerializer):
    class Meta:
        model = TblHeaderPkts
        fields = "__all__"

    