const colorSelector = document.querySelector(".row.gtr-50.gtr-uniform");
const firstColor = document.querySelector(".first-color");
const secondColor = document.querySelector(".second-color");
const thirdColor = document.querySelector(".third-color");
const foodRecoBtn = document.querySelector("#food-reco-btn");

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
foodRecoBtn.addEventListener('click', () => startSendColor());


function startSendColor(){ 
    let jsonTest = new Array();
    // JSON 
    let selectColorList = new Object() ;
    // 사용자가 선택한 color 헥사값 저장 리스트
    let colorHexList = new Array();
    // food 리스트 
    let foodList = new Array();


    // color key에 저장될 값
    colorHexList.push(firstColor.innerTex.substring(1, 6));
    colorHexList.push(secondColor.innerText.substring(1, 6));
    colorHexList.push(thirdColor.innerTex.substring(1, 6));

    // foods key에 저장될 값
    foodList.push("굴");

    // Object의 id속성 생성
    selectColorList.colors = colorHexList;
    selectColorList.foods = foodList;

    jsonTest.push(selectColorList);

    // String 형태로 변환
    let jsonData = JSON.stringify(jsonTest) ;

    alert(jsonData) ;

    $.ajax({ 
        url: 'http://1.246.129.141/recommendation', //images.json
        type: 'POST', 
        dataType: 'json',
        data: jsonData,
        ContentType: 'application/json',
        success: function(result){
            console.log('POST 성공');
            alert(jsonData) ;
        },
        error: function(){ 
            alert("ERROR") ;
        }
    }); 
} 
