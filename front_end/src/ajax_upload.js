    //Mypage의 사진 업로드
    $.ajax({
        url: "http://addr.of.server/upload_image",
        method:"POST",
        success: function(result){
            if (result){
                console.log(result) 
                
            }else{
                alert("불러오기 실패");
            }
        }
    })
    function dataSend() { 
        var data=$("#input").val(); 
        
        var messageDTO={ 
            result:data 
        }; 
        
        $.ajax({ 
            url: "/dataSend", 
            data: messageDTO, 
            type:"POST", 
        }).done(function (fragment) { 
            $("#resultDiv").replaceWith(fragment); 
        }); 
    }
