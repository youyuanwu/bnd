// Test fixture: references a struct type defined in another header
// that is NOT in the traverse list. Should trigger validation error.

struct KnownStruct {
    int x;
    int y;
};

// MissingStruct is forward-declared but never defined — this produces
// CType::Void (incomplete record), which is fine.
struct MissingForwardDecl;

#include "unresolved_dep.h"

// This function takes a DefinedElsewhere* parameter — since the struct
// IS complete (defined in unresolved_dep.h), clang produces
// CType::Named { name: "DefinedElsewhere", resolved: None }.
// But if we don't traverse unresolved_dep.h, the type won't be in
// the registry and validation should catch it.
int use_external(struct DefinedElsewhere* thing);
int use_known(struct KnownStruct* thing);
