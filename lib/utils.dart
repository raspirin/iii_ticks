import 'package:flutter/material.dart';

class VerticalSplitView extends StatefulWidget {
  final Widget left;
  final Widget right;
  final double ratio;

  const VerticalSplitView(
      {required this.left, required this.right, this.ratio = 0.5})
      : assert(ratio >= 0),
        assert(ratio <= 1);

  @override
  _VerticalSplitViewState createState() => _VerticalSplitViewState();
}

class _VerticalSplitViewState extends State<VerticalSplitView> {
  // final _dividerWidth = 16.0;

  //from 0-1
  double _ratio = 0.5;
  double _maxWidth = 200;

  get _width1 => _ratio * _maxWidth;

  get _width2 => (1 - _ratio) * _maxWidth;

  @override
  void initState() {
    super.initState();
    _ratio = widget.ratio;
  }

  @override
  Widget build(BuildContext context) {
    return LayoutBuilder(builder: (context, BoxConstraints constraints) {
      assert(_ratio <= 1);
      assert(_ratio >= 0);
      _maxWidth = constraints.maxWidth;
      if (_maxWidth != constraints.maxWidth) {
        _maxWidth = constraints.maxWidth;
      }

      return SizedBox(
        width: constraints.maxWidth,
        child: Row(
          children: <Widget>[
            SizedBox(
              width: _width1,
              child: widget.left,
            ),
            // GestureDetector(
            //   behavior: HitTestBehavior.translucent,
            //   child: SizedBox(
            //     width: _dividerWidth,
            //     height: constraints.maxHeight,
            //     child: const RotationTransition(
            //       turns: AlwaysStoppedAnimation(0.25),
            //       child: Icon(Icons.drag_handle),
            //     ),
            //   ),
            //   onPanUpdate: (DragUpdateDetails details) {
            //     setState(() {
            //       _ratio += details.delta.dx / _maxWidth;
            //       if (_ratio > 1) {
            //         _ratio = 1;
            //       } else if (_ratio < 0.0) {
            //         _ratio = 0.0;
            //       }
            //     });
            //   },
            // ),
            SizedBox(
              width: _width2,
              child: widget.right,
            ),
          ],
        ),
      );
    });
  }
}

class AppButton extends StatelessWidget {
  final String data;
  final EdgeInsetsGeometry padding;
  const AppButton(
      {super.key, required this.data, this.padding = const EdgeInsets.all(8)});
  @override
  Widget build(BuildContext context) {
    return TextButton(
        onPressed: () {},
        style: ButtonStyle(
            shape: MaterialStateProperty.all(RoundedRectangleBorder(
                borderRadius: BorderRadius.circular(6.0))),
            padding: MaterialStateProperty.all(padding),
            backgroundColor:
                MaterialStateProperty.all(Theme.of(context).primaryColorDark),
            minimumSize: MaterialStateProperty.all(Size.zero)),
        child: Text(
          data,
          textAlign: TextAlign.center,
          style: Theme.of(context)
              .textTheme
              .displaySmall!
              .copyWith(fontSize: 16, fontFamily: "MonomaniacOne"),
        ));
  }
}
