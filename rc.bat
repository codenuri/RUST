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
