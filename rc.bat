# rc.bat
# 1. rustc 컴파일러로 컴파일을 하고 ( -A, -o 옵션 적용 )
# 2. 생성된 out.exe 파일을 실행 하는 배치 파일
#
# 사용법 1 : rc sample.rs
# 사용법 2 : rc sample.rs -C opt-level=3    처럼 추가 옵션 전달 가능
#
# github.com/codenuri/RUST 에서 다운로드 가능

cls

@echo [101mrustc %* -A dead_code -A unused_variables -A unused_assignments -o out.exe [0m
@rustc %* -A dead_code -A unused_variables -A unused_assignments -o out.exe

@IF %ERRORLEVEL% EQU 0 (
	echo [37;4m[ rustc compiling success !! run out.exe ][0m
	echo.
  	out.exe
) ELSE (
	echo [37;4m[ Error !!, rustc fail to compile. ][0m
)

@echo.
