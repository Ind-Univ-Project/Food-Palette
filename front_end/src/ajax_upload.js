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
   