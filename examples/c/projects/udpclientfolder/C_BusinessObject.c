#include "C_BusinessObject.h"

C_BusinessObject *in_C_BusinessObject(C_BusinessObject *self)
{
  self->mallocated = 0;
  self->mythings = 0;
  self->deleteself_C_BusinessObject = &deleteself_C_BusinessObject;
  self->somefunction_C_BusinessObject = &somefunction_C_BusinessObject;
  return self;
}
C_BusinessObject *nw_C_BusinessObject()
{
  C_BusinessObject *self = malloc(sizeof(C_BusinessObject));
  self = in_C_BusinessObject(self);
  self->mallocated = 1; // Is in dynamic memory
  return self;
}
void deleteself_C_BusinessObject(C_BusinessObject * self)
{
  // First delete owned objects 
  // self->otherObj->deleteself(self->otherObj);
  //
  if (self->mallocated)
  {
    free(self);
  }
  return;
};
void somefunction_C_BusinessObject(C_BusinessObject * self)
{
  ;
}
