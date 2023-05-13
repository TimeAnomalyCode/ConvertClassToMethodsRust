template <class T>
class Container{
	private:
		T value;
		
	public:
		Container();
		T getValue() const;
		void setValue(T n);
};