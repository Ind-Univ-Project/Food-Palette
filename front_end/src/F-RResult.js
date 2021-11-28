window.onload = function () { 
    console.log('LOAD 완료!!')
    startLoadFile(); 
}; 

function startLoadFile(){ 
    $.ajax({ 
        url: 'http://1.246.129.141/image/d9562bca-afe0-4f9a-8587-f17cd9b54e0f.png', 
        type: 'GET', 
        dataType : 'image/? ', 
        success : function (data) { 
            if(data) {
                console.log('GET 완료!!');
                
                createImages(data);
            } else {
                console.log('GET Fail');
            }
        }
    }); 
} 

// JSON 포멧 데이터 처리 
function createImages(objImageInfo) { 
    let image = objImageInfo;
    let imageHTML = '';
    const test = document.querySelector('div.gtr-50');

    // GET으로 받아온 image 출력
    imageHTMl += `<img src="${image}">`;
    test.append(imageHTML);

    // var images = objImageInfo.images; 
    // var strDOM = ""; 
    
    // for (var i = 0; i < images.length; i++) { 
    //     // N번째 이미지 정보를 구하기 
    //     var image = images[i]; 
    //     // N번째 이미지 패널을 생성
    //     strDOM += '<div class="image_panel">'; 
    //     strDOM += '<img src="' + image.url + '">'; 
    //     strDOM += '<p class="title">' + image.title +'</p>'; 
    //     strDOM += '</div>'; 
    // }

    // // 이미지 컨테이너에 생성한 이미지 패널들을 추가하기 
    // var $imageContainer = $("#image_container"); 
    // $imageContainer.append(strDOM); 
}

