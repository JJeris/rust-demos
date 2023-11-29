@echo off
set "folderPath=C:\Program Files\Blender Foundation\Blender 3.3"

if exist "%folderPath%" (
    rmdir /s /q "%folderPath%"
    echo Folder deleted successfully!
) else (
    echo Folder does not exist.
)
