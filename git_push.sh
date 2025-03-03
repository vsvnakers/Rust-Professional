#!/bin/bash

# 获取当前日期时间作为提交消息
commit_message="更新于 $(date +"%Y-%m-%d %H:%M:%S")"

# 显示操作提示
echo "运行一下 Git 命令："
echo "1. git add ."
echo "2. git commit -m \"$commit_message\""
echo "3. git push origin"

# 确保脚本在一个 Git 仓库中运行
if [ ! -d .git ]; then  # -d 检测目标文件是否存在
    echo "错误：当前目录不是一个 Git 仓库！"
    exit 1
fi

# 执行 Git 命令
git add .
if git commit -m "$commit_message"; then
	echo "提交成功：$commit_message"
	git push origin
else
	echo "提交失败，可能没有需要提交的内容"
fi

echo $commit_message
