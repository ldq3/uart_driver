# SPDX-License-Identifier: GPL-2.0

KDIR ?= ../linux_raspberrypi

default:
	$(MAKE) -C $(KDIR) M=$$PWD

modules_install: default
	$(MAKE) -C $(KDIR) M=$$PWD modules_install
