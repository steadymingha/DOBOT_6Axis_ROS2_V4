# generated from rosidl_generator_py/resource/_idl.py.em
# with input from dobot_msgs_v4:srv/GetDOGroup.idl
# generated code does not contain a copyright notice


# Import statements for member types

# Member 'index_group'
import array  # noqa: E402, I100

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_GetDOGroup_Request(type):
    """Metaclass of message 'GetDOGroup_Request'."""

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
                'dobot_msgs_v4.srv.GetDOGroup_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__get_do_group__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__get_do_group__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__get_do_group__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__get_do_group__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__get_do_group__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class GetDOGroup_Request(metaclass=Metaclass_GetDOGroup_Request):
    """Message class 'GetDOGroup_Request'."""

    __slots__ = [
        '_index_group',
    ]

    _fields_and_field_types = {
        'index_group': 'sequence<int32>',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.UnboundedSequence(rosidl_parser.definition.BasicType('int32')),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.index_group = array.array('i', kwargs.get('index_group', []))

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
        if self.index_group != other.index_group:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def index_group(self):
        """Message field 'index_group'."""
        return self._index_group

    @index_group.setter
    def index_group(self, value):
        if isinstance(value, array.array):
            assert value.typecode == 'i', \
                "The 'index_group' array.array() must have the type code of 'i'"
            self._index_group = value
            return
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
                 all(isinstance(v, int) for v in value) and
                 all(val >= -2147483648 and val < 2147483648 for val in value)), \
                "The 'index_group' field must be a set or sequence and each value of type 'int' and each integer in [-2147483648, 2147483647]"
        self._index_group = array.array('i', value)


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_GetDOGroup_Response(type):
    """Metaclass of message 'GetDOGroup_Response'."""

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
                'dobot_msgs_v4.srv.GetDOGroup_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__get_do_group__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__get_do_group__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__get_do_group__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__get_do_group__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__get_do_group__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class GetDOGroup_Response(metaclass=Metaclass_GetDOGroup_Response):
    """Message class 'GetDOGroup_Response'."""

    __slots__ = [
        '_robot_return',
        '_res',
    ]

    _fields_and_field_types = {
        'robot_return': 'string',
        'res': 'int32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.robot_return = kwargs.get('robot_return', str())
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
        if self.robot_return != other.robot_return:
            return False
        if self.res != other.res:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def robot_return(self):
        """Message field 'robot_return'."""
        return self._robot_return

    @robot_return.setter
    def robot_return(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'robot_return' field must be of type 'str'"
        self._robot_return = value

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


class Metaclass_GetDOGroup(type):
    """Metaclass of service 'GetDOGroup'."""

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
                'dobot_msgs_v4.srv.GetDOGroup')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__get_do_group

            from dobot_msgs_v4.srv import _get_do_group
            if _get_do_group.Metaclass_GetDOGroup_Request._TYPE_SUPPORT is None:
                _get_do_group.Metaclass_GetDOGroup_Request.__import_type_support__()
            if _get_do_group.Metaclass_GetDOGroup_Response._TYPE_SUPPORT is None:
                _get_do_group.Metaclass_GetDOGroup_Response.__import_type_support__()


class GetDOGroup(metaclass=Metaclass_GetDOGroup):
    from dobot_msgs_v4.srv._get_do_group import GetDOGroup_Request as Request
    from dobot_msgs_v4.srv._get_do_group import GetDOGroup_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
