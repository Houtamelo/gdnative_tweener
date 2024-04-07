START /B /wait cargo build --release -F integration_tests

echo off

timeout /t 1

echo on

copy /Y /B "%~dp0target\release\gdnative_tweener.dll" "%~dp0tester\Bin\gdnative_tweener.dll"

pause