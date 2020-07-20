#include <stdio.h>
#include "classskeleton.h"


 /* Object in static memory */
 XYZ global_XYZ;
 XYZ *global_XYZ_Ptr;

int main(int argc, char** argv)
{
  /* Object on stack */
  XYZ stack_XYZ;
  XYZ *stack_XYZ_Ptr;

  global_XYZ_Ptr = in_XYZ(&global_XYZ);
  stack_XYZ_Ptr = in_XYZ(&stack_XYZ);

  /* Object in heap */ 
  XYZ *heap_XYZ_Ptr;
  heap_XYZ_Ptr = nw_XYZ();

  int result_glob = global_XYZ_Ptr->XYZ_method1 (global_XYZ_Ptr,34,56);
  int result_stack = stack_XYZ_Ptr->XYZ_method1 (stack_XYZ_Ptr,34,56);
  int result_heap = heap_XYZ_Ptr->XYZ_method1 (heap_XYZ_Ptr,34,56);

  if (global_XYZ_Ptr->something == 3)
    ;
  if (stack_XYZ_Ptr->something == 3)
    ;
  if (heap_XYZ_Ptr->something == 3)
    ;

  // XYZ_method1 is hidden and static. Must use ptr->mth(prt,..);

  global_XYZ_Ptr -> XYZ_method1(global_XYZ_Ptr,23,45);

  // XYZ_method1 is public. mth(prt,..)
  // Shorter code but dangerous. 
  XYZ_method2(global_XYZ_Ptr,23);
  
  XYZ_deleteself(&global_XYZ_Ptr);
  XYZ_deleteself(&stack_XYZ_Ptr);
  XYZ_deleteself(&heap_XYZ_Ptr);
  return 1;
  }