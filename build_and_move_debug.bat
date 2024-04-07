START /B /wait cargo build -F integration_tests

echo off

timeout /t 1

echo on

copy /Y /B "%~dp0target\debug\gdnative_tweener.dll" "%~dp0tester\Bin\gdnative_tweener.dll"

pause
