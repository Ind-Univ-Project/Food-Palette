const colorSelector = document.querySelector(".row.gtr-50.gtr-uniform");
const firstColor = document.querySelector(".first-color");
const secondColor = document.querySelector(".second-color");
const thirdColor = document.querySelector(".third-color");

let value1;

function colorClick(event) {
    const dataSet = event.target.dataset;

    // 예시
    value1 = dataSet.value;

    if (firstColor.innerText == '- 1st -') {
        firstColor.innerText = dataSet.value;
    }
    else if (firstColor.innerText != '- 1st -' && secondColor.innerText == '- 2nd -') {
        secondColor.innerText = dataSet.value;
    }
    else if (secondColor.innerText != '- 2nd -' && thirdColor.innerText == '- 3rd -' ) {
        thirdColor.innerText = dataSet.value;
    }
}

colorSelector.addEventListener('click', event => colorClick(event));

function startSendColor(){ 
    // 사용자가 선택한 1순위~3순위의 컬러값을 저장
    var selectColorList = new Array() ;

    for(let i = 0; i < 3; i++) {
        // 객체 생성
        let data = new Object() ;
        
        data.color = i ;
        data.name = "Color #" + i ;
        
        
        // 리스트에 생성된 객체 삽입
        testList.push(data) ;
    }
    
    // String 형태로 변환
    var jsonData = JSON.stringify(testList) ;
    
    alert(jsonData) ;


    $.ajax({ 
        url: 'http://1.246.129.141/', //images.json
        type: 'POST', 
        data: {
            'color1': firstColor, 
            'color2': secondColor, 
            'color3' : thirdColor
        },
        success: function(result){
        },
        error:function(){  
        }
    }); 
} 