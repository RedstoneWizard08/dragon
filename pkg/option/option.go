package option

import "errors"

type Option[T any] struct {
	value *T
}

func Some[T any](value T) Option[T] {
	return Option[T]{value: &value}
}

func None[T any]() Option[T] {
	return Option[T]{}
}

func (opt *Option[T]) IsNone() bool {
	return opt.value == nil
}

func (opt *Option[T]) IsSome() bool {
	return !opt.IsNone()
}

func (opt *Option[T]) Unwrap() (*T, error) {
	if opt.IsNone() {
		return nil, errors.New("Tried to unwrap a nil value!")
	}

	return opt.value, nil
}

func (opt *Option[T]) UnwrapOr(other T) T {
	if opt.IsNone() {
		return other
	}

	return *opt.value
}

func (opt *Option[T]) IfPresent(fn func(val T)) Option[T] {
	if opt.IsSome() {
		fn(*opt.value)
	}

	return *opt
}

func Map[I any, O any](opt Option[I], mapper func(val I) O) (Option[O], error) {
	val, err := opt.Unwrap()

	if err != nil {
		return None[O](), err
	}

	res := mapper(*val)

	return Some[O](res), nil
}
