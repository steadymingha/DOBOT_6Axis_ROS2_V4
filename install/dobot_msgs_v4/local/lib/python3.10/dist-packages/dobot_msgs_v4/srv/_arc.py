# generated from rosidl_generator_py/resource/_idl.py.em
# with input from dobot_msgs_v4:srv/Arc.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_Arc_Request(type):
    """Metaclass of message 'Arc_Request'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('dobot_msgs_v4')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'dobot_msgs_v4.srv.Arc_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__arc__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__arc__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__arc__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__arc__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__arc__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class Arc_Request(metaclass=Metaclass_Arc_Request):
    """Message class 'Arc_Request'."""

    __slots__ = [
        '_mode',
        '_a',
        '_b',
        '_c',
        '_d',
        '_e',
        '_f',
        '_a2',
        '_b2',
        '_c2',
        '_d2',
        '_e2',
        '_f2',
        '_param_value',
    ]

    _fields_and_field_types = {
        'mode': 'boolean',
        'a': 'double',
        'b': 'double',
        'c': 'double',
        'd': 'double',
        'e': 'double',
        'f': 'double',
        'a2': 'double',
        'b2': 'double',
        'c2': 'double',
        'd2': 'double',
        'e2': 'double',
        'f2': 'double',
        'param_value': 'sequence<string>',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.UnboundedSequence(rosidl_parser.definition.UnboundedString()),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.mode = kwargs.get('mode', bool())
        self.a = kwargs.get('a', float())
        self.b = kwargs.get('b', float())
        self.c = kwargs.get('c', float())
        self.d = kwargs.get('d', float())
        self.e = kwargs.get('e', float())
        self.f = kwargs.get('f', float())
        self.a2 = kwargs.get('a2', float())
        self.b2 = kwargs.get('b2', float())
        self.c2 = kwargs.get('c2', float())
        self.d2 = kwargs.get('d2', float())
        self.e2 = kwargs.get('e2', float())
        self.f2 = kwargs.get('f2', float())
        self.param_value = kwargs.get('param_value', [])

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.mode != other.mode:
            return False
        if self.a != other.a:
            return False
        if self.b != other.b:
            return False
        if self.c != other.c:
            return False
        if self.d != other.d:
            return False
        if self.e != other.e:
            return False
        if self.f != other.f:
            return False
        if self.a2 != other.a2:
            return False
        if self.b2 != other.b2:
            return False
        if self.c2 != other.c2:
            return False
        if self.d2 != other.d2:
            return False
        if self.e2 != other.e2:
            return False
        if self.f2 != other.f2:
            return False
        if self.param_value != other.param_value:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def mode(self):
        """Message field 'mode'."""
        return self._mode

    @mode.setter
    def mode(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'mode' field must be of type 'bool'"
        self._mode = value

    @builtins.property
    def a(self):
        """Message field 'a'."""
        return self._a

    @a.setter
    def a(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'a' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'a' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._a = value

    @builtins.property
    def b(self):
        """Message field 'b'."""
        return self._b

    @b.setter
    def b(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'b' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'b' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._b = value

    @builtins.property
    def c(self):
        """Message field 'c'."""
        return self._c

    @c.setter
    def c(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'c' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'c' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._c = value

    @builtins.property
    def d(self):
        """Message field 'd'."""
        return self._d

    @d.setter
    def d(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'd' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'd' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._d = value

    @builtins.property
    def e(self):
        """Message field 'e'."""
        return self._e

    @e.setter
    def e(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'e' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'e' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._e = value

    @builtins.property
    def f(self):
        """Message field 'f'."""
        return self._f

    @f.setter
    def f(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'f' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'f' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._f = value

    @builtins.property
    def a2(self):
        """Message field 'a2'."""
        return self._a2

    @a2.setter
    def a2(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'a2' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'a2' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._a2 = value

    @builtins.property
    def b2(self):
        """Message field 'b2'."""
        return self._b2

    @b2.setter
    def b2(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'b2' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'b2' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._b2 = value

    @builtins.property
    def c2(self):
        """Message field 'c2'."""
        return self._c2

    @c2.setter
    def c2(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'c2' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'c2' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._c2 = value

    @builtins.property
    def d2(self):
        """Message field 'd2'."""
        return self._d2

    @d2.setter
    def d2(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'd2' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'd2' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._d2 = value

    @builtins.property
    def e2(self):
        """Message field 'e2'."""
        return self._e2

    @e2.setter
    def e2(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'e2' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'e2' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._e2 = value

    @builtins.property
    def f2(self):
        """Message field 'f2'."""
        return self._f2

    @f2.setter
    def f2(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'f2' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'f2' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._f2 = value

    @builtins.property
    def param_value(self):
        """Message field 'param_value'."""
        return self._param_value

    @param_value.setter
    def param_value(self, value):
        if __debug__:
            from collections.abc import Sequence
            from collections.abc import Set
            from collections import UserList
            from collections import UserString
            assert \
                ((isinstance(value, Sequence) or
                  isinstance(value, Set) or
                  isinstance(value, UserList)) and
                 not isinstance(value, str) and
                 not isinstance(value, UserString) and
                 all(isinstance(v, str) for v in value) and
                 True), \
                "The 'param_value' field must be a set or sequence and each value of type 'str'"
        self._param_value = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_Arc_Response(type):
    """Metaclass of message 'Arc_Response'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('dobot_msgs_v4')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'dobot_msgs_v4.srv.Arc_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__arc__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__arc__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__arc__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__arc__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__arc__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class Arc_Response(metaclass=Metaclass_Arc_Response):
    """Message class 'Arc_Response'."""

    __slots__ = [
        '_res',
    ]

    _fields_and_field_types = {
        'res': 'int32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.res = kwargs.get('res', int())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.res != other.res:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def res(self):
        """Message field 'res'."""
        return self._res

    @res.setter
    def res(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'res' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'res' field must be an integer in [-2147483648, 2147483647]"
        self._res = value


class Metaclass_Arc(type):
    """Metaclass of service 'Arc'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('dobot_msgs_v4')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'dobot_msgs_v4.srv.Arc')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__arc

            from dobot_msgs_v4.srv import _arc
            if _arc.Metaclass_Arc_Request._TYPE_SUPPORT is None:
                _arc.Metaclass_Arc_Request.__import_type_support__()
            if _arc.Metaclass_Arc_Response._TYPE_SUPPORT is None:
                _arc.Metaclass_Arc_Response.__import_type_support__()


class Arc(metaclass=Metaclass_Arc):
    from dobot_msgs_v4.srv._arc import Arc_Request as Request
    from dobot_msgs_v4.srv._arc import Arc_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
