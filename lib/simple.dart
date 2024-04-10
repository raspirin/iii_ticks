import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/widgets.dart';
import 'package:iii_ticks/utils.dart';

class SimpleMain extends StatefulWidget {
  const SimpleMain({super.key});

  @override
  State<StatefulWidget> createState() {
    return _SimpleMainState();
  }
}

class _SimpleMainState extends State<SimpleMain> {
  double bpm = 100;
  double gain = 0;

  @override
  Widget build(BuildContext context) {
    return VerticalSplitView(
        left: Container(
          // color: Theme.of(context).colorScheme.background,
          decoration: BoxDecoration(
              color: Theme.of(context).colorScheme.background,
              border: Border(
                  right: BorderSide(
                      color: Theme.of(context).colorScheme.onSecondary))),

          child: Center(
            child: Wrap(
              alignment: WrapAlignment.center,
              crossAxisAlignment: WrapCrossAlignment.center,
              spacing: 0,
              direction: Axis.vertical,
              children: <Widget>[
                Text(
                  bpm.floor().toString(),
                  style: Theme.of(context)
                      .primaryTextTheme
                      .titleLarge!
                      .copyWith(fontSize: 92, fontWeight: FontWeight.w300),
                ),
                Transform.translate(
                  offset: const Offset(0, -10),
                  child: Text(
                    "BPM",
                    style: Theme.of(context)
                        .primaryTextTheme
                        .titleMedium!
                        .copyWith(
                          fontSize: 36,
                          fontWeight: FontWeight.w200,
                        ),
                  ),
                ),
                const BpmChange(),
                const Padding(
                  padding: EdgeInsets.all(12.0),
                  child: GainChange(),
                )
              ],
            ),
          ),
        ),
        right: Container(
          color: Theme.of(context).colorScheme.background,
          child: Text("World!"),
        ));
  }
}

class GainChange extends StatelessWidget {
  const GainChange({super.key});

  @override
  Widget build(BuildContext context) {
    return Wrap(
      direction: Axis.horizontal,
      spacing: 12,
      children: [
        const AppButton(
          data: "-",
          padding: EdgeInsets.all(4),
        ),
        Text(
          "-3dB",
          style: Theme.of(context)
              .primaryTextTheme
              .displaySmall!
              .copyWith(fontSize: 18),
        ), //todo
        const AppButton(
          data: "+",
          padding: EdgeInsets.all(4),
        )
      ],
    );
  }
}

class BpmChange extends StatelessWidget {
  const BpmChange({super.key});

  @override
  Widget build(BuildContext context) {
    return const Wrap(
      direction: Axis.horizontal,
      spacing: 10,
      children: [
        BpmButton(change: -10),
        BpmButton(change: -1),
        BpmButton(change: 1),
        BpmButton(change: 10),
      ],
    );
  }
}

class BpmButton extends StatelessWidget {
  final int change;
  const BpmButton({super.key, required this.change});
  @override
  Widget build(BuildContext context) {
    String sign = change > 0 ? "+" : "-";
    return AppButton(
      data: "$sign\n${change.abs()}",
      padding: const EdgeInsets.symmetric(vertical: 12, horizontal: 8),
    );
  }
}
