@echo off
setlocal enabledelayedexpansion
chcp 65001 > nul

REM 检查是否提供了标签版本参数
if "%~1"=="" (
    echo 错误：请提供标签版本作为参数。
    echo 用法：%0 标签版本
    exit /b 1
)

REM 设置标签版本
set TAG_VERSION=%~1

REM 删除远程标签
echo 正在删除远程标签 %TAG_VERSION%...
git push origin --delete tag %TAG_VERSION%

REM 删除本地标签
echo 正在删除本地标签 %TAG_VERSION%...
git tag --delete %TAG_VERSION%

REM 重新创建本地标签
echo 正在重新创建本地标签 %TAG_VERSION%...
git tag %TAG_VERSION%

REM 推送新标签到远程仓库
echo 正在推送新标签 %TAG_VERSION% 到远程仓库...
git push origin --tags

echo 完成！