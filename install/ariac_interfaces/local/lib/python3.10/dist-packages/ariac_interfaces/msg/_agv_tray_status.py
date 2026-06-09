# generated from rosidl_generator_py/resource/_idl.py.em
# with input from ariac_interfaces:msg/AgvTrayStatus.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_AgvTrayStatus(type):
    """Metaclass of message 'AgvTrayStatus'."""

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
                'ariac_interfaces.msg.AgvTrayStatus')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__agv_tray_status
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__agv_tray_status
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__agv_tray_status
            cls._TYPE_SUPPORT = module.type_support_msg__msg__agv_tray_status
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__agv_tray_status

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class AgvTrayStatus(metaclass=Metaclass_AgvTrayStatus):
    """Message class 'AgvTrayStatus'."""

    __slots__ = [
        '_slot_1_occupied',
        '_slot_2_occupied',
        '_slot_3_occupied',
        '_slot_4_occupied',
    ]

    _fields_and_field_types = {
        'slot_1_occupied': 'boolean',
        'slot_2_occupied': 'boolean',
        'slot_3_occupied': 'boolean',
        'slot_4_occupied': 'boolean',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
        rosidl_parser.definition.BasicType('boolean'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.slot_1_occupied = kwargs.get('slot_1_occupied', bool())
        self.slot_2_occupied = kwargs.get('slot_2_occupied', bool())
        self.slot_3_occupied = kwargs.get('slot_3_occupied', bool())
        self.slot_4_occupied = kwargs.get('slot_4_occupied', bool())

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
        if self.slot_1_occupied != other.slot_1_occupied:
            return False
        if self.slot_2_occupied != other.slot_2_occupied:
            return False
        if self.slot_3_occupied != other.slot_3_occupied:
            return False
        if self.slot_4_occupied != other.slot_4_occupied:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def slot_1_occupied(self):
        """Message field 'slot_1_occupied'."""
        return self._slot_1_occupied

    @slot_1_occupied.setter
    def slot_1_occupied(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'slot_1_occupied' field must be of type 'bool'"
        self._slot_1_occupied = value

    @builtins.property
    def slot_2_occupied(self):
        """Message field 'slot_2_occupied'."""
        return self._slot_2_occupied

    @slot_2_occupied.setter
    def slot_2_occupied(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'slot_2_occupied' field must be of type 'bool'"
        self._slot_2_occupied = value

    @builtins.property
    def slot_3_occupied(self):
        """Message field 'slot_3_occupied'."""
        return self._slot_3_occupied

    @slot_3_occupied.setter
    def slot_3_occupied(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'slot_3_occupied' field must be of type 'bool'"
        self._slot_3_occupied = value

    @builtins.property
    def slot_4_occupied(self):
        """Message field 'slot_4_occupied'."""
        return self._slot_4_occupied

    @slot_4_occupied.setter
    def slot_4_occupied(self, value):
        if __debug__:
            assert \
                isinstance(value, bool), \
                "The 'slot_4_occupied' field must be of type 'bool'"
        self._slot_4_occupied = value
