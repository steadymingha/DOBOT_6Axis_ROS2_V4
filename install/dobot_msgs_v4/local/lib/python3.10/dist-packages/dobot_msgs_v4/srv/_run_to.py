# generated from rosidl_generator_py/resource/_idl.py.em
# with input from dobot_msgs_v4:srv/RunTo.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_RunTo_Request(type):
    """Metaclass of message 'RunTo_Request'."""

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
                'dobot_msgs_v4.srv.RunTo_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__run_to__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__run_to__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__run_to__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__run_to__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__run_to__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'USER__DEFAULT': -1,
            'TOOL__DEFAULT': -1,
            'A__DEFAULT': -1,
            'V__DEFAULT': -1,
        }

    @property
    def USER__DEFAULT(cls):
        """Return default value for message field 'user'."""
        return -1

    @property
    def TOOL__DEFAULT(cls):
        """Return default value for message field 'tool'."""
        return -1

    @property
    def A__DEFAULT(cls):
        """Return default value for message field 'a'."""
        return -1

    @property
    def V__DEFAULT(cls):
        """Return default value for message field 'v'."""
        return -1


class RunTo_Request(metaclass=Metaclass_RunTo_Request):
    """Message class 'RunTo_Request'."""

    __slots__ = [
        '_a1',
        '_b1',
        '_c1',
        '_d1',
        '_e1',
        '_f1',
        '_move_type',
        '_user',
        '_tool',
        '_a',
        '_v',
    ]

    _fields_and_field_types = {
        'a1': 'double',
        'b1': 'double',
        'c1': 'double',
        'd1': 'double',
        'e1': 'double',
        'f1': 'double',
        'move_type': 'int32',
        'user': 'int32',
        'tool': 'int32',
        'a': 'int32',
        'v': 'int32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.a1 = kwargs.get('a1', float())
        self.b1 = kwargs.get('b1', float())
        self.c1 = kwargs.get('c1', float())
        self.d1 = kwargs.get('d1', float())
        self.e1 = kwargs.get('e1', float())
        self.f1 = kwargs.get('f1', float())
        self.move_type = kwargs.get('move_type', int())
        self.user = kwargs.get(
            'user', RunTo_Request.USER__DEFAULT)
        self.tool = kwargs.get(
            'tool', RunTo_Request.TOOL__DEFAULT)
        self.a = kwargs.get(
            'a', RunTo_Request.A__DEFAULT)
        self.v = kwargs.get(
            'v', RunTo_Request.V__DEFAULT)

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
        if self.a1 != other.a1:
            return False
        if self.b1 != other.b1:
            return False
        if self.c1 != other.c1:
            return False
        if self.d1 != other.d1:
            return False
        if self.e1 != other.e1:
            return False
        if self.f1 != other.f1:
            return False
        if self.move_type != other.move_type:
            return False
        if self.user != other.user:
            return False
        if self.tool != other.tool:
            return False
        if self.a != other.a:
            return False
        if self.v != other.v:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def a1(self):
        """Message field 'a1'."""
        return self._a1

    @a1.setter
    def a1(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'a1' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'a1' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._a1 = value

    @builtins.property
    def b1(self):
        """Message field 'b1'."""
        return self._b1

    @b1.setter
    def b1(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'b1' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'b1' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._b1 = value

    @builtins.property
    def c1(self):
        """Message field 'c1'."""
        return self._c1

    @c1.setter
    def c1(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'c1' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'c1' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._c1 = value

    @builtins.property
    def d1(self):
        """Message field 'd1'."""
        return self._d1

    @d1.setter
    def d1(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'd1' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'd1' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._d1 = value

    @builtins.property
    def e1(self):
        """Message field 'e1'."""
        return self._e1

    @e1.setter
    def e1(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'e1' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'e1' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._e1 = value

    @builtins.property
    def f1(self):
        """Message field 'f1'."""
        return self._f1

    @f1.setter
    def f1(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'f1' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'f1' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._f1 = value

    @builtins.property
    def move_type(self):
        """Message field 'move_type'."""
        return self._move_type

    @move_type.setter
    def move_type(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'move_type' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'move_type' field must be an integer in [-2147483648, 2147483647]"
        self._move_type = value

    @builtins.property
    def user(self):
        """Message field 'user'."""
        return self._user

    @user.setter
    def user(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'user' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'user' field must be an integer in [-2147483648, 2147483647]"
        self._user = value

    @builtins.property
    def tool(self):
        """Message field 'tool'."""
        return self._tool

    @tool.setter
    def tool(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'tool' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'tool' field must be an integer in [-2147483648, 2147483647]"
        self._tool = value

    @builtins.property
    def a(self):
        """Message field 'a'."""
        return self._a

    @a.setter
    def a(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'a' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'a' field must be an integer in [-2147483648, 2147483647]"
        self._a = value

    @builtins.property
    def v(self):
        """Message field 'v'."""
        return self._v

    @v.setter
    def v(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'v' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'v' field must be an integer in [-2147483648, 2147483647]"
        self._v = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_RunTo_Response(type):
    """Metaclass of message 'RunTo_Response'."""

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
                'dobot_msgs_v4.srv.RunTo_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__run_to__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__run_to__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__run_to__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__run_to__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__run_to__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class RunTo_Response(metaclass=Metaclass_RunTo_Response):
    """Message class 'RunTo_Response'."""

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


class Metaclass_RunTo(type):
    """Metaclass of service 'RunTo'."""

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
                'dobot_msgs_v4.srv.RunTo')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__run_to

            from dobot_msgs_v4.srv import _run_to
            if _run_to.Metaclass_RunTo_Request._TYPE_SUPPORT is None:
                _run_to.Metaclass_RunTo_Request.__import_type_support__()
            if _run_to.Metaclass_RunTo_Response._TYPE_SUPPORT is None:
                _run_to.Metaclass_RunTo_Response.__import_type_support__()


class RunTo(metaclass=Metaclass_RunTo):
    from dobot_msgs_v4.srv._run_to import RunTo_Request as Request
    from dobot_msgs_v4.srv._run_to import RunTo_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
