#ifndef C_BusinessObject_H
#define C_BusinessObject_H

#include <stdlib.h>


typedef struct C_BusinessObject C_BusinessObject;

void deleteself_C_BusinessObject(C_BusinessObject * self);
void somefunction_C_BusinessObject(C_BusinessObject * self);

struct C_BusinessObject {
  int mallocated;
	int mythings;
  void (*deleteself_C_BusinessObject)(C_BusinessObject * self);
  void (*somefunction_C_BusinessObject)(C_BusinessObject * self);
};

void deleteself_C_BusinessObject(C_BusinessObject * self);

C_BusinessObject * in_C_BusinessObject(C_BusinessObject * self);
C_BusinessObject * nw_C_BusinessObject();

#endif /* C_BusinessObject_H */

