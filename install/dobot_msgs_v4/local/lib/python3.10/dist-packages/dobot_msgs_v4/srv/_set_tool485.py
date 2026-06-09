# generated from rosidl_generator_py/resource/_idl.py.em
# with input from dobot_msgs_v4:srv/SetTool485.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_SetTool485_Request(type):
    """Metaclass of message 'SetTool485_Request'."""

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
                'dobot_msgs_v4.srv.SetTool485_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__set_tool485__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__set_tool485__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__set_tool485__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__set_tool485__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__set_tool485__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class SetTool485_Request(metaclass=Metaclass_SetTool485_Request):
    """Message class 'SetTool485_Request'."""

    __slots__ = [
        '_baudrate',
        '_parity',
        '_stop',
        '_identify',
    ]

    _fields_and_field_types = {
        'baudrate': 'int32',
        'parity': 'string',
        'stop': 'int32',
        'identify': 'int32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.baudrate = kwargs.get('baudrate', int())
        self.parity = kwargs.get('parity', str())
        self.stop = kwargs.get('stop', int())
        self.identify = kwargs.get('identify', int())

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
        if self.baudrate != other.baudrate:
            return False
        if self.parity != other.parity:
            return False
        if self.stop != other.stop:
            return False
        if self.identify != other.identify:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def baudrate(self):
        """Message field 'baudrate'."""
        return self._baudrate

    @baudrate.setter
    def baudrate(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'baudrate' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'baudrate' field must be an integer in [-2147483648, 2147483647]"
        self._baudrate = value

    @builtins.property
    def parity(self):
        """Message field 'parity'."""
        return self._parity

    @parity.setter
    def parity(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'parity' field must be of type 'str'"
        self._parity = value

    @builtins.property
    def stop(self):
        """Message field 'stop'."""
        return self._stop

    @stop.setter
    def stop(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'stop' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'stop' field must be an integer in [-2147483648, 2147483647]"
        self._stop = value

    @builtins.property
    def identify(self):
        """Message field 'identify'."""
        return self._identify

    @identify.setter
    def identify(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'identify' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'identify' field must be an integer in [-2147483648, 2147483647]"
        self._identify = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_SetTool485_Response(type):
    """Metaclass of message 'SetTool485_Response'."""

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
                'dobot_msgs_v4.srv.SetTool485_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__set_tool485__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__set_tool485__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__set_tool485__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__set_tool485__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__set_tool485__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class SetTool485_Response(metaclass=Metaclass_SetTool485_Response):
    """Message class 'SetTool485_Response'."""

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


class Metaclass_SetTool485(type):
    """Metaclass of service 'SetTool485'."""

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
                'dobot_msgs_v4.srv.SetTool485')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__set_tool485

            from dobot_msgs_v4.srv import _set_tool485
            if _set_tool485.Metaclass_SetTool485_Request._TYPE_SUPPORT is None:
                _set_tool485.Metaclass_SetTool485_Request.__import_type_support__()
            if _set_tool485.Metaclass_SetTool485_Response._TYPE_SUPPORT is None:
                _set_tool485.Metaclass_SetTool485_Response.__import_type_support__()


class SetTool485(metaclass=Metaclass_SetTool485):
    from dobot_msgs_v4.srv._set_tool485 import SetTool485_Request as Request
    from dobot_msgs_v4.srv._set_tool485 import SetTool485_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
