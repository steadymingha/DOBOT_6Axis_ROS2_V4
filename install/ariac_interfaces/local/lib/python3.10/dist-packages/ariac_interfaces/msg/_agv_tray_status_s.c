// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from ariac_interfaces:msg/AgvTrayStatus.idl
// generated code does not contain a copyright notice
#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
#include <Python.h>
#include <stdbool.h>
#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-function"
#endif
#include "numpy/ndarrayobject.h"
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif
#include "rosidl_runtime_c/visibility_control.h"
#include "ariac_interfaces/msg/detail/agv_tray_status__struct.h"
#include "ariac_interfaces/msg/detail/agv_tray_status__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool ariac_interfaces__msg__agv_tray_status__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[52];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("ariac_interfaces.msg._agv_tray_status.AgvTrayStatus", full_classname_dest, 51) == 0);
  }
  ariac_interfaces__msg__AgvTrayStatus * ros_message = _ros_message;
  {  // slot_1_occupied
    PyObject * field = PyObject_GetAttrString(_pymsg, "slot_1_occupied");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->slot_1_occupied = (Py_True == field);
    Py_DECREF(field);
  }
  {  // slot_2_occupied
    PyObject * field = PyObject_GetAttrString(_pymsg, "slot_2_occupied");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->slot_2_occupied = (Py_True == field);
    Py_DECREF(field);
  }
  {  // slot_3_occupied
    PyObject * field = PyObject_GetAttrString(_pymsg, "slot_3_occupied");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->slot_3_occupied = (Py_True == field);
    Py_DECREF(field);
  }
  {  // slot_4_occupied
    PyObject * field = PyObject_GetAttrString(_pymsg, "slot_4_occupied");
    if (!field) {
      return false;
    }
    assert(PyBool_Check(field));
    ros_message->slot_4_occupied = (Py_True == field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * ariac_interfaces__msg__agv_tray_status__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of AgvTrayStatus */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("ariac_interfaces.msg._agv_tray_status");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "AgvTrayStatus");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  ariac_interfaces__msg__AgvTrayStatus * ros_message = (ariac_interfaces__msg__AgvTrayStatus *)raw_ros_message;
  {  // slot_1_occupied
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->slot_1_occupied ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "slot_1_occupied", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // slot_2_occupied
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->slot_2_occupied ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "slot_2_occupied", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // slot_3_occupied
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->slot_3_occupied ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "slot_3_occupied", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // slot_4_occupied
    PyObject * field = NULL;
    field = PyBool_FromLong(ros_message->slot_4_occupied ? 1 : 0);
    {
      int rc = PyObject_SetAttrString(_pymessage, "slot_4_occupied", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}
