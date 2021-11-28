    //Mypage의 사진 업로드
    let file = document.querySelector('input#Upload');
    let base64_result;

    let reader = new FileReader(file);
    // base64 encoding(maybe..)
    reader.onload = function() {
        // 파일이 정상적으로 read -> result = 'encoding값 저장 & 반환'
        base64_result = reader.result;
        console.log(base64_result);
    }

    // 실패할 경우, error 출력하기
    reader.onerror = function() {
        console.log('Error');
    }

    $('#UploadImage').on('click', function(event) {
        event.preventDefalut();

        let form = $('#uploadForm')[1];
        let date = new FormData(form);

        $.ajax({
            url: "http://1.246.129.141/upload_image",
            type:"POST",
            enctype: 'multipart/form-data',
            data: {
                category: '연어초밥',
                image_type: 'png',
                image_buffer: ""
            },
            success: function(result){
                if (result){
                    console.log(result) 
                    console.log("성공했다!!!!!") 
                    
                }else{
                    alert("불러오기 실패");
                }
            }
        })

    })
    
   
    //F-R Result 이미지 이름으로 요청
    $.ajax({
        url: "http://address.of.server/image/image_name",
        method:"GET",
        success: function(result){
            if (result){
                document.getElementById("food-image").src = result
            }else{
                alert("불러오기 실패");
            }

        }
    })
    //
    $.ajax({
        url: "http://address.of.server/image/image_name",
        method:"GET",
        success: function(result){
            if (result){
                document.getElementById("food-image").src = result
            }else{
                alert("불러오기 실패");
            }

        }
    })