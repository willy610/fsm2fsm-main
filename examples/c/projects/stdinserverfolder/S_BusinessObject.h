#ifndef S_BusinessObject_H
#define S_BusinessObject_H

#include <stdlib.h>

typedef struct S_BusinessObject S_BusinessObject;

void deleteself_S_BusinessObject(S_BusinessObject * self);
void somefunction_S_BusinessObject(S_BusinessObject * self);

struct S_BusinessObject {
  int mallocated;
	int mythings;
  void (*deleteself_S_BusinessObject)(S_BusinessObject * self);
  void (*somefunction_S_BusinessObject)(S_BusinessObject * self);
};

void deleteself_S_BusinessObject(S_BusinessObject * self);

S_BusinessObject * in_S_BusinessObject(S_BusinessObject * self);
S_BusinessObject * nw_S_BusinessObject(void);

#endif /* S_BusinessObject_H */

