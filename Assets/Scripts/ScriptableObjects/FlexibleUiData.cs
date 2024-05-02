using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;
using UnityEngine.Serialization;
using UnityEngine.TextCore.Text;
using UnityEngine.UI;

[CreateAssetMenu( menuName = "UiData/Base Ui Data" )]
public class FlexibleUiData : ScriptableObject
{
    public Sprite hpBarFill;
    public Sprite hpBarBackground;

    public TMP_FontAsset font;
}