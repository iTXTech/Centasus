# iTXTech Centasus

**开源，跨平台，原生的储存器校验工具，为`U盘`和`固态硬盘`打造。**

## 前端

* 用于实现用户交互
* 计划实现以下前端
  * 终端 - 即 `Console`
  * 图形化 - 即 `Desktop`，将基于 `flutter-rs`

## 后端

* 储存设备枚举
* `ODTD2` 文件访问

## `ODTD2` 文件

* `ODTD2` 即 `OpenDriveTesterData2`
-----
* `ODTD2` 是 `Centasus` 生成的用于完整性校验的文件
* 每个`ODTD2`文件对应一个**固定**的`Seed`，详见源代码
* `ODTD2` 文件仅包含随机生成的数据，不含任何其它头部信息

## 许可证

    Copyright (C) 2020 iTX Technologies

	This program is free software: you can redistribute it and/or modify
	it under the terms of the GNU General Public License as published by
	the Free Software Foundation, either version 3 of the License, or
	(at your option) any later version.

	This program is distributed in the hope that it will be useful,
	but WITHOUT ANY WARRANTY; without even the implied warranty of
	MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
	GNU General Public License for more details.

	You should have received a copy of the GNU General Public License
	along with this program.  If not, see <http://www.gnu.org/licenses/>.
