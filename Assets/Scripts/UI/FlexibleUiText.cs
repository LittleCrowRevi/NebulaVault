using System.Collections;
using System.Collections.Generic;
using TMPro;
using UnityEngine;

[RequireComponent(typeof(RectTransform))]
[RequireComponent(typeof(CanvasRenderer))]
[RequireComponent(typeof(TMP_Text))]
public class FlexibleUiText : FlexibleUi
{
    [Header( "Type Data" )]
    public ScriptableObject observedData;

    public override void Update()
    {
        base.Update();
        
        var text = GetComponent< TMP_Text >();
        if ( !text ) return;

        if ( skinData && skinData.font ) text.font = skinData.font;

        if ( !observedData ) return;
        switch ( observedData )
        {
            case PoolSO data:
                text.text = $"HP {data.currentValue}/{data.baseValue}";
                break;

            case IntVariable data:
                text.text = $"{data.RuntimeValue}";
                break;
        }
    }
}