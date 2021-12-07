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


