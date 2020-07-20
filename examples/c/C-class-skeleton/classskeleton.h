#ifndef CLASSSKELETON_H
#define CLASSSKELETON_H

#ifdef __cplusplus
extern "C" {
#endif

	typedef struct XYZ XYZ;

  struct XYZ {
    int malloceted;
    void (*XYZ_deleteself)(XYZ ** self);
    // User attributes
    int something;
    // User methods
    int (*XYZ_method1)(XYZ * self, int p1, int p2);
    int (*XYZ_method2)(XYZ * self, int p1);
  };
	// Class Constructor methods
	// For Instances as static or stack 
	XYZ * in_XYZ(XYZ * self);
	// For Instances in dynamic memory
	XYZ * nw_XYZ();

	// Object or Instance methods
	
	void XYZ_deleteself(XYZ ** self);

  // User methods
  // static is hidden
	static int XYZ_method1(XYZ * self, int p1, int p2);
  // none statci is public
	int XYZ_method2(XYZ * self, int p1);

#ifdef __cplusplus
}
#endif

#endif /* CLASSSKELETON_H */

