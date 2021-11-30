<img alt="GitHub commit activity" src="https://img.shields.io/github/commit-activity/w/Ind-Univ-Project/Food-Palette">  <img alt="GitHub milestones" src="https://img.shields.io/github/milestones/all/Ind-Univ-Project/Food-Palette">  <img alt="GitHub milestones" src="https://img.shields.io/badge/license-MIT-brightgreene">
# Food Palette
Food Palette는 유저가 입력한 색을 기반으로 유저에게 적절한 음식 사진을 추천하는 웹 기반 서비스 입니다.

## Install Guide
Food-Palette는 웹 기반 서비스로 별도의 유저 클라이언트를 설치할 필요가 없습니다.

## How to Use
Food-Palette 의 서버 구성은 Web-Server와 API-Server로 나뉘어져 있으며
해당 시스템은 별도의 환경에서 실행될 수 있습니다

Web-Server는 일반적인 웹 서버 어플리케이션에서 실행할 수 있는 코드로 되어있어
해당 코드를 웹 서버의 적절한 디렉터리에 배치하여 실행할 수 있습니다

API-Server는 Rust언어로 되어있어, cargo의 설치 이후
적절한 빌드 과정을 거치면 바로 사용할 수 있습니다


## Dependency
관리자용 이미지 업로드 도구
https://github.com/Ind-Univ-Project/Image-Auto-Uploader

예시용 이미지 데이터셋
https://github.com/Ind-Univ-Project/Image-Extractor
