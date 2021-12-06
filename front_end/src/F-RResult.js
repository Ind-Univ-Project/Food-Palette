// window.onload = function () { 
//     console.log('LOAD 완료!!')
//     startLoadFile(); 
// }; 

// function startLoadFile(){ 
//     $.ajax({ 
//         url: 'http://1.246.129.141/image/d9562bca-afe0-4f9a-8587-f17cd9b54e0f.png', //images.json
//         type: 'GET', 
//         dataType : 'json', //jsonp
//         //jsonp: 'jsonpCallBack',
//         success : function (data) { 
//             if(data) { //성공메시지
//                 console.log('GET 완료!!');
//                 createImages(data);
//             } else { //실패 메시지
//                 console.log('GET Fail');
//             }
//         }
//     }); 
// } 



// // JSON 포멧 데이터 처리 
// function createImages(objImageInfo) { 
//     // let image = objImageInfo.images; //objImageInfo.images;
//     // let imageHTML = '';
//     // const test = document.querySelector('div.gtr-50');

//     // // GET으로 받아온 image 출력
//     // imageHTMl += `<img src="${image}">`;
//     // test.append(imageHTML);

//     var images = objImageInfo.images; 
//     var strDOM = ""; 
    
//     for (var i = 0; i < images.length; i++) { 
//         // N번째 이미지 정보를 구하기 
//         var image = images[i]; 
//         // N번째 이미지 패널을 생성
//         strDOM += '<div class="image_panel">'; 
//         strDOM += '<img src="' + image.url + '">'; 
//         strDOM += '<p class="title">' + image.title +'</p>'; 
//         strDOM += '</div>'; 
//     }

//     // 이미지 컨테이너에 생성한 이미지 패널들을 추가하기 
//     var $imageContainer = $("#image_container"); 
//     $imageContainer.append(strDOM); 
// }

// var a = $("#result1")
//     .attr("href", "https://user-images.githubusercontent.com/53327279/143773625-20a83454-af56-4de9-b218-619c8da44355.png")
//     .attr("src", "https://user-images.githubusercontent.com/53327279/143773625-20a83454-af56-4de9-b218-619c8da44355.png")
//     // .attr("download", "img.png")
    
//     .appendTo("#result1");

// //a[0].click();

// a.remove();

// // //팝업처럼 사진만 뜬다.
// // $(function (){
// //     $("#result1").hover(
// //         function(){
// //             $(this).attr("src","https://user-images.githubusercontent.com/53327279/143773625-20a83454-af56-4de9-b218-619c8da44355.png");
// //         }
// //     );
// // });

// $("#result1").attr("src", "https://user-images.githubusercontent.com/53327279/143773625-20a83454-af56-4de9-b218-619c8da44355.png")
// $("#result2").attr("src", "https://user-images.githubusercontent.com/53327279/143773092-513cd5a1-60f7-4fd6-9ad6-33dfc24e514c.png")
// $("#result3").attr("src", "https://user-images.githubusercontent.com/53327279/143771553-ddd1c1c8-5526-416a-9209-9533bd2b14a1.png")

// GET 방식으로 서버에 HTTP Request를 보냄. 
$.get("http://1.246.129.141/", 
// 서버가 필요한 정보를 같이 보냄. 
    { color1: "yellow", color2: "red", color3: "blue", }, 
        function(data, status) { 
            $("#result1").html(data + "<br>" + status); 
            
// 전송받은 데이터와 전송 성공 여부를 보여줌. 
        } 
    ); 


function startLoadFile(){ 
    $.ajax({ 
        url: 'http://1.246.129.141/', //images.json
        type: 'GET', 
        dataType : 'json', //jsonp
        //jsonp: 'jsonpCallBack',
        success : function (data) { 
            if(data) { //성공메시지
                console.log('GET 완료!!');

                const result = document.querySelector('.Result .gtr-50');
                let html = "";

                for(let i = 0; i < 3; i++) {
                    html += `
                        <p>${i+1}. ${data[i][0]}
                            <div class='col-12-small'>
                                <span class="image fit">
                                    <img alt="result${i+1}" id="result${i+1}" style="display: block; margin: 0 auto; max-width: 50%; height: auto;"/>
                                </span>
                            </div>
                        </p>
                    `;
                }
                
                result.innerHTML = html;

                $("#result1").attr("src", url + data[0][1]); 
                $("#result2").attr("src", url + data[1][1]); 
                $("#result2").attr("src", url + data[2][1]); 
                createImages(data);
            } else { //실패 메시지
                console.log('GET Fail');
            }
        }
    }); 
} 