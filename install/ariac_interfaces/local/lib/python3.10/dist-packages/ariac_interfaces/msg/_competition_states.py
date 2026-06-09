# generated from rosidl_generator_py/resource/_idl.py.em
# with input from ariac_interfaces:msg/CompetitionStates.idl
# generated code does not contain a copyright notice


# Import statements for member types

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_CompetitionStates(type):
    """Metaclass of message 'CompetitionStates'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'PREPARING': 0,
        'READY': 1,
        'STARTED': 2,
        'ORDERS_COMPLETE': 3,
        'ENDED': 4,
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
                'ariac_interfaces.msg.CompetitionStates')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__competition_states
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__competition_states
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__competition_states
            cls._TYPE_SUPPORT = module.type_support_msg__msg__competition_states
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__competition_states

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'PREPARING': cls.__constants['PREPARING'],
            'READY': cls.__constants['READY'],
            'STARTED': cls.__constants['STARTED'],
            'ORDERS_COMPLETE': cls.__constants['ORDERS_COMPLETE'],
            'ENDED': cls.__constants['ENDED'],
        }

    @property
    def PREPARING(self):
        """Message constant 'PREPARING'."""
        return Metaclass_CompetitionStates.__constants['PREPARING']

    @property
    def READY(self):
        """Message constant 'READY'."""
        return Metaclass_CompetitionStates.__constants['READY']

    @property
    def STARTED(self):
        """Message constant 'STARTED'."""
        return Metaclass_CompetitionStates.__constants['STARTED']

    @property
    def ORDERS_COMPLETE(self):
        """Message constant 'ORDERS_COMPLETE'."""
        return Metaclass_CompetitionStates.__constants['ORDERS_COMPLETE']

    @property
    def ENDED(self):
        """Message constant 'ENDED'."""
        return Metaclass_CompetitionStates.__constants['ENDED']


class CompetitionStates(metaclass=Metaclass_CompetitionStates):
    """
    Message class 'CompetitionStates'.

    Constants:
      PREPARING
      READY
      STARTED
      ORDERS_COMPLETE
      ENDED
    """

    __slots__ = [
    ]

    _fields_and_field_types = {
    }

    SLOT_TYPES = (
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))

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
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)
