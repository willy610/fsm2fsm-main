#include <stdlib.h>
#include <stdio.h>
#include "classskeleton.h"
 
 
static int other_XYZ_method1(XYZ * self, int p1, int p2);
 
XYZ * in_XYZ(XYZ * self)
{
  self->malloceted =0;
  self->XYZ_deleteself = &XYZ_deleteself;

  // Overkill ??
  // You can set up different function implementations. Like verify / prod.
  // User functions
  if (1==1)
    self->XYZ_method1 = &XYZ_method1;
  else
    self->XYZ_method1 = &other_XYZ_method1;
    
  self->XYZ_method2 = &XYZ_method2;
  // init user attributes
  self->something = 0;
  
  return self;
}
XYZ * nw_XYZ()
{
  XYZ * self = malloc(sizeof (XYZ));
  self = in_XYZ(self);
  self->malloceted =1; // Is in dynamic memory
  return self;
}
void XYZ_deleteself(XYZ ** self)
{
  if ((*self)->malloceted)
  {
    free(*self);
  }
  *self = 0;
  return;
};
static int XYZ_method1(XYZ * self, int p1, int p2)
{
  printf("int XYZ_method1 called\n");
  self->something += p1;
  return self->something +p2;
};
static int other_XYZ_method1(XYZ * self, int p1, int p2)
{
  printf("other_XYZ_method1 called\n");
  self->something += p1;
  return self->something +p2;
};

int XYZ_method2(XYZ * self, int p1)
{ 
  printf("static int XYZ_method2 called\n");
  return self->something + p1;
};
