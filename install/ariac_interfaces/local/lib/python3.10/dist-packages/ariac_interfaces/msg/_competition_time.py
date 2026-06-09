# generated from rosidl_generator_py/resource/_idl.py.em
# with input from ariac_interfaces:msg/CompetitionTime.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_CompetitionTime(type):
    """Metaclass of message 'CompetitionTime'."""

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
            module = import_type_support('ariac_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ariac_interfaces.msg.CompetitionTime')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__competition_time
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__competition_time
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__competition_time
            cls._TYPE_SUPPORT = module.type_support_msg__msg__competition_time
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__competition_time

            from builtin_interfaces.msg import Duration
            if Duration.__class__._TYPE_SUPPORT is None:
                Duration.__class__.__import_type_support__()

            from builtin_interfaces.msg import Time
            if Time.__class__._TYPE_SUPPORT is None:
                Time.__class__.__import_type_support__()

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class CompetitionTime(metaclass=Metaclass_CompetitionTime):
    """Message class 'CompetitionTime'."""

    __slots__ = [
        '_start',
        '_elapsed',
        '_remaining',
    ]

    _fields_and_field_types = {
        'start': 'builtin_interfaces/Time',
        'elapsed': 'builtin_interfaces/Duration',
        'remaining': 'builtin_interfaces/Duration',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.NamespacedType(['builtin_interfaces', 'msg'], 'Time'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['builtin_interfaces', 'msg'], 'Duration'),  # noqa: E501
        rosidl_parser.definition.NamespacedType(['builtin_interfaces', 'msg'], 'Duration'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        from builtin_interfaces.msg import Time
        self.start = kwargs.get('start', Time())
        from builtin_interfaces.msg import Duration
        self.elapsed = kwargs.get('elapsed', Duration())
        from builtin_interfaces.msg import Duration
        self.remaining = kwargs.get('remaining', Duration())

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
        if self.start != other.start:
            return False
        if self.elapsed != other.elapsed:
            return False
        if self.remaining != other.remaining:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def start(self):
        """Message field 'start'."""
        return self._start

    @start.setter
    def start(self, value):
        if __debug__:
            from builtin_interfaces.msg import Time
            assert \
                isinstance(value, Time), \
                "The 'start' field must be a sub message of type 'Time'"
        self._start = value

    @builtins.property
    def elapsed(self):
        """Message field 'elapsed'."""
        return self._elapsed

    @elapsed.setter
    def elapsed(self, value):
        if __debug__:
            from builtin_interfaces.msg import Duration
            assert \
                isinstance(value, Duration), \
                "The 'elapsed' field must be a sub message of type 'Duration'"
        self._elapsed = value

    @builtins.property
    def remaining(self):
        """Message field 'remaining'."""
        return self._remaining

    @remaining.setter
    def remaining(self, value):
        if __debug__:
            from builtin_interfaces.msg import Duration
            assert \
                isinstance(value, Duration), \
                "The 'remaining' field must be a sub message of type 'Duration'"
        self._remaining = value
