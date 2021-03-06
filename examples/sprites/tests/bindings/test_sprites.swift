import sprites

let sempty = Sprite(initialPosition: nil)
assert( sempty.getPosition() == Point(x: 0, y: 0))

let s = Sprite(initialPosition: Point(x: 0, y: 1))
assert( s.getPosition() == Point(x: 0, y: 1))

s.moveTo(position: Point(x: 1.0, y: 2.0))
assert( s.getPosition() == Point(x: 1, y: 2))

s.moveBy(direction: Vector(dx: -4, dy: 2))
assert( s.getPosition() == Point(x: -3, y: 4))


