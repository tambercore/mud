@echo off
echo Creating virtual environment 'lambeq_env'...
call python -m venv ..\data\lambeq\lambeq_env

echo Activating virtual environment...
call ..\data\lambeq\lambeq_env\Scripts\activate

echo Installing dependencies from requirements.txt...
pip install --upgrade pip
pip install -r ..\data\lambeq\requirements.txt

echo Virtual environment 'lambeq_env' is ready.
pause
