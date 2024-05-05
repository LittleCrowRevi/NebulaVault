using System;
using System.Collections;
using System.Collections.Generic;
using System.Text;
using TMPro;
using UnityEngine;

[RequireComponent( typeof( RectTransform ) )]
[RequireComponent( typeof( CanvasRenderer ) )]
[RequireComponent( typeof( TMP_Text ) )]
public class FlexibleUiText : FlexibleUi
{
    [Header( "Type Data" )]
    public ScriptableObject observedData;

    [ConditionalProperty( "observedData", typeof( EntityData ) )]
    [Tooltip( "Stat Type to observe." )]
    public StatType statType;

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

            case EntityData data:
                var dataString = new StringBuilder();
                switch ( statType )
                {
                    case StatType.Focus:
                        dataString.Append( data.focus );
                        break;

                    case StatType.Mind:
                        dataString.Append( data.mind );
                        break;

                    case StatType.Body:
                        dataString.Append( data.body );
                        break;
                }

                text.text = $"{dataString}";
                break;
        }
    }
}