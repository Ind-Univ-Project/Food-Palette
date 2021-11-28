window.onload = function () { 
    startLoadFile(); 
}; 
function startLoadFile(){ 
    $.ajax({ 
        url: 'images.json', 
        type: 'GET', 
        dataType : 'json', 
        success : function (data) { 
            createImages(data) 
        } 
    }); 
} 

// JSON 포멧 데이터 처리 
function createImages(objImageInfo) { 
    var images = objImageInfo.images; 
    var strDOM = ""; 
    
    for (var i = 0; i < images.length; i++) { 
        // N번째 이미지 정보를 구하기 
        var image = images[i]; 
        // N번째 이미지 패널을 생성
        strDOM += '<div class="image_panel">'; 
        strDOM += '<img src="' + image.url + '">'; 
        strDOM += '<p class="title">' + image.title +'</p>'; 
        strDOM += '</div>'; 
    }

    // 이미지 컨테이너에 생성한 이미지 패널들을 추가하기 
    var $imageContainer = $("#image_container"); 
    $imageContainer.append(strDOM); 
}

