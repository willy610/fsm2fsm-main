#include "S_BusinessObject.h"

S_BusinessObject *in_S_BusinessObject(S_BusinessObject *self)
{
  self->mallocated = 0;
  self->mythings = 0;
  self->deleteself_S_BusinessObject = &deleteself_S_BusinessObject;
  self->somefunction_S_BusinessObject = &somefunction_S_BusinessObject;
  return self;
}
S_BusinessObject *nw_S_BusinessObject()
{
  S_BusinessObject *self = malloc(sizeof(S_BusinessObject));
  self = in_S_BusinessObject(self);
  self->mallocated = 1; // Is in dynamic memory
  return self;
}
void deleteself_S_BusinessObject(S_BusinessObject * self)
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
void somefunction_S_BusinessObject(S_BusinessObject * self)
{
  ;
}
