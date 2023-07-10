#include "wrapper.h"

PIO_STACK_LOCATION _IoGetCurrentIrpStackLocation(PIRP irp) {
	return IoGetCurrentIrpStackLocation(irp);
}

void _KeQuerySystemTime(PLARGE_INTEGER CurrentTime) {
	return KeQuerySystemTime(CurrentTime);
}


NTKERNELAPI PVOID NTAPI PsGetProcessWow64Process(_In_ PEPROCESS Process);