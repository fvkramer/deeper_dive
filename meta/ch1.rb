class MyClass
  def self.define_component(name)
    define_method(name) do
      "I am a #{name}"
    end
  end

  define_component :monkey
  define_component :meta
  define_component :snake
end

test = MyClass.new

p test.monkey
p test.meta
p test.snake