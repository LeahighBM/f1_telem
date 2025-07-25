from django.shortcuts import render
from django.http import HttpResponse, JsonResponse
from django.views.decorators.csrf import csrf_exempt
from rest_framework.parsers import JSONParser
from telem_app.models import TblHeaderPkts
from telem_app.serializers import HeaderSerializer

# Create your views here.
@csrf_exempt
def header_list(request):
    if request.method == "GET":
        headers = TblHeaderPkts.objects.all();
        serializer = HeaderSerializer(headers, many=True)
        return JsonResponse(serializer.data, safe=False)
    elif request.method == "POST":
        data = JSONParser().parse(request)
        print(data)
        headers = TblHeaderPkts.objects.filter(time__gte=data.get("start"), 
                                               time__lte=data.get("stop"))
        serializer = HeaderSerializer(headers, many=True)
        return JsonResponse(serializer.data, safe=False)
    else:
        return HttpResponse(status=405, content=f"Method {request.method} not allowed")
    

@csrf_exempt
def single_header(request, id):
    try:
        header = TblHeaderPkts.objects.get(pk=id)
    except TblHeaderPkts.DoesNotExist:
        return HttpResponse(status=404)
    
    print(repr(header))
    
    if request.method == "GET":
        serializer = HeaderSerializer(header)
        return JsonResponse(serializer.data)
    
